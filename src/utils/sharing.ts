import { invoke } from "@tauri-apps/api/core";

async function getClipboardManager() {
  return import("@tauri-apps/plugin-clipboard-manager");
}

export async function encodeSharedState(data: string): Promise<string> {
  const encoder = new TextEncoder();
  const input = encoder.encode(data);
  const cs = new CompressionStream("gzip");
  const writer = cs.writable.getWriter();
  writer.write(input);
  writer.close();
  const reader = cs.readable.getReader();
  const chunks: Uint8Array[] = [];
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
  }
  const blob = new Blob(chunks as BlobPart[]);
  const buffer = await blob.arrayBuffer();
  const bytes = new Uint8Array(buffer);
  let binary = "";
  for (let i = 0; i < bytes.length; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}

export async function decodeSharedState(base64: string): Promise<string> {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  const ds = new DecompressionStream("gzip");
  const writer = ds.writable.getWriter();
  writer.write(bytes);
  writer.close();
  const reader = ds.readable.getReader();
  const chunks: Uint8Array[] = [];
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
  }
  const decoder = new TextDecoder();
  return (
    chunks.map((c) => decoder.decode(c, { stream: true })).join("") +
    decoder.decode()
  );
}

export async function shareState(): Promise<void> {
  const json = await invoke<string>("export_state");
  const compressed = await encodeSharedState(json);
  const { writeText } = await getClipboardManager();
  await writeText(compressed);
}

export async function importShared(): Promise<void> {
  const { readText } = await getClipboardManager();
  const compressed = await readText();
  if (!compressed) {
    throw new Error("Clipboard does not contain shared OrbitForge state.");
  }
  const json = await decodeSharedState(compressed);
  await invoke("import_state", { json });
}
