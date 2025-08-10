import { atom, useAtom } from "jotai";

export type AuthState = true | false | "offline";

export const authenticatedAtom = atom<AuthState>(false);

export function useAuth() {
  const [auth, setAuth] = useAtom(authenticatedAtom);

  const login = () => setAuth(true);
  const logout = () => setAuth(false);
  const goOffline = () => setAuth("offline");

  return { auth, login, logout, goOffline };
}
