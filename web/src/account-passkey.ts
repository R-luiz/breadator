// Passkeys (WebAuthn): registering one from the account page and signing in with one. The server
// (crates/breadator-server/src/auth/passkey.rs) speaks JSON with base64url for every binary field;
// the browser API wants ArrayBuffers, so this file converts both ways.
import { t } from "./i18n";

export function passkeysSupported(): boolean {
  return typeof window.PublicKeyCredential === "function" && typeof navigator.credentials?.create === "function";
}

function fromBase64Url(value: string): ArrayBuffer {
  const padded = value.replace(/-/g, "+").replace(/_/g, "/").padEnd(Math.ceil(value.length / 4) * 4, "=");
  const binary = atob(padded);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return bytes.buffer;
}

function toBase64Url(buffer: ArrayBuffer): string {
  let binary = "";
  for (const byte of new Uint8Array(buffer)) binary += String.fromCharCode(byte);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

interface Descriptor {
  id: string;
  transports?: AuthenticatorTransport[];
}

/** The `publicKey` options as the server sends them: binary fields are base64url strings. */
interface PublicKeyJson {
  challenge: string;
  user?: { id: string; name: string; displayName: string };
  excludeCredentials?: Descriptor[];
  allowCredentials?: Descriptor[];
}

interface CeremonyJson {
  ceremony: string;
  publicKey: PublicKeyJson;
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

/** `{ ceremony, options: { publicKey } }` from the start endpoints, or null if it is not that. */
function parseCeremony(json: Record<string, unknown>): CeremonyJson | null {
  const { ceremony, options } = json;
  if (typeof ceremony !== "string" || !isObject(options) || !isObject(options.publicKey)) return null;
  if (typeof options.publicKey.challenge !== "string") return null;
  // The server built these options with the fields the browser API needs; only the binary
  // fields are converted below.
  const publicKey = options.publicKey as unknown as PublicKeyJson;
  return { ceremony, publicKey };
}

function descriptors(list: Descriptor[] | undefined): PublicKeyCredentialDescriptor[] | undefined {
  return list?.map((c) => ({ type: "public-key", id: fromBase64Url(c.id), transports: c.transports }));
}

/** A failure the page can word: a catalog key for the message. */
export class PasskeyError extends Error {
  constructor(readonly key: string) {
    super(key);
  }
}

async function post(path: string, body: unknown): Promise<{ status: number; json: Record<string, unknown> }> {
  const response = await fetch(path, {
    method: "POST",
    credentials: "same-origin",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
  const json: unknown = await response.json().catch(() => ({}));
  return { status: response.status, json: isObject(json) ? json : {} };
}

function errorCode(json: Record<string, unknown>): string {
  const { error } = json;
  return isObject(error) && typeof error.code === "string" ? error.code : "";
}

/** Asks the server for a challenge, lets the authenticator create the credential, stores it. */
export async function registerPasskey(name: string): Promise<void> {
  if (!passkeysSupported()) throw new PasskeyError("auth-passkey-error-unsupported");
  const start = await post("/api/v1/passkeys/register/start", {});
  const ceremony = start.status === 200 ? parseCeremony(start.json) : null;
  const pk = ceremony?.publicKey;
  if (!ceremony || !pk?.user) {
    throw new PasskeyError(errorCode(start.json) === "too_many_passkeys" ? "account-passkey-error-limit" : "account-passkey-error-failed");
  }
  // Same object with the binary fields converted: the compiler cannot see through the spread.
  const options = {
    ...pk,
    challenge: fromBase64Url(pk.challenge),
    user: { ...pk.user, id: fromBase64Url(pk.user.id) },
    excludeCredentials: descriptors(pk.excludeCredentials),
  } as unknown as PublicKeyCredentialCreationOptions;
  let credential: Credential | null;
  try {
    credential = await navigator.credentials.create({ publicKey: options });
  } catch (error) {
    // The authenticator already holds a credential of this account: it refuses to make another.
    if (error instanceof DOMException && error.name === "InvalidStateError") {
      throw new PasskeyError("account-passkey-error-duplicate");
    }
    if (error instanceof DOMException && error.name === "NotAllowedError") {
      throw new PasskeyError("auth-passkey-error-cancelled");
    }
    throw new PasskeyError("account-passkey-error-failed");
  }
  if (!(credential instanceof PublicKeyCredential) || !(credential.response instanceof AuthenticatorAttestationResponse)) {
    throw new PasskeyError("auth-passkey-error-cancelled");
  }
  const finish = await post("/api/v1/passkeys/register/finish", {
    ceremony: ceremony.ceremony,
    name,
    credential: {
      id: credential.id,
      rawId: toBase64Url(credential.rawId),
      type: credential.type,
      response: {
        attestationObject: toBase64Url(credential.response.attestationObject),
        clientDataJSON: toBase64Url(credential.response.clientDataJSON),
      },
      extensions: credential.getClientExtensionResults(),
    },
  });
  if (finish.status !== 200) {
    throw new PasskeyError(
      errorCode(finish.json) === "already_registered" ? "account-passkey-error-duplicate" : "account-passkey-error-failed",
    );
  }
}

/** Signs in with a discoverable credential (no username typed); returns where to go next. */
export async function signInWithPasskey(): Promise<string> {
  if (!passkeysSupported()) throw new PasskeyError("auth-passkey-error-unsupported");
  const start = await post("/api/v1/passkeys/login/start", {});
  const ceremony = start.status === 200 ? parseCeremony(start.json) : null;
  if (!ceremony) throw new PasskeyError(start.status === 429 ? "auth-too-many-title" : "auth-passkey-error-failed");
  const { publicKey: pk } = ceremony;
  // The server's options ask for conditional mediation (autofill); a button press is the modal
  // form, so only `publicKey` is passed on.
  const options = {
    ...pk,
    challenge: fromBase64Url(pk.challenge),
    allowCredentials: descriptors(pk.allowCredentials),
  } as unknown as PublicKeyCredentialRequestOptions;
  let credential: Credential | null;
  try {
    credential = await navigator.credentials.get({ publicKey: options });
  } catch (error) {
    if (error instanceof DOMException && error.name === "NotAllowedError") {
      throw new PasskeyError("auth-passkey-error-cancelled");
    }
    throw new PasskeyError("auth-passkey-error-failed");
  }
  if (!(credential instanceof PublicKeyCredential) || !(credential.response instanceof AuthenticatorAssertionResponse)) {
    throw new PasskeyError("auth-passkey-error-cancelled");
  }
  const { response } = credential;
  const finish = await post("/api/v1/passkeys/login/finish", {
    ceremony: ceremony.ceremony,
    credential: {
      id: credential.id,
      rawId: toBase64Url(credential.rawId),
      type: credential.type,
      response: {
        authenticatorData: toBase64Url(response.authenticatorData),
        clientDataJSON: toBase64Url(response.clientDataJSON),
        signature: toBase64Url(response.signature),
        userHandle: response.userHandle ? toBase64Url(response.userHandle) : null,
      },
      extensions: credential.getClientExtensionResults(),
    },
  });
  if (finish.status !== 200) {
    throw new PasskeyError(finish.status === 429 ? "auth-too-many-title" : "auth-passkey-error-failed");
  }
  const { redirect } = finish.json;
  return typeof redirect === "string" && redirect.startsWith("/") && !redirect.startsWith("//") ? redirect : "/";
}

export function describePasskeyError(error: unknown): string {
  return t(error instanceof PasskeyError ? error.key : "auth-passkey-error-failed");
}
