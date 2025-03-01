import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

export default function Greet() {
  const [name, setName] = useState("");
  const [msg, setMsg] = useState("");

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const resp = await invoke<string>("greet", { name });
    setMsg(resp);
  };

  return (
    <>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <button type="submit">Greet</button>
      </form>
      <p>{msg}</p>
    </>
  );
}
