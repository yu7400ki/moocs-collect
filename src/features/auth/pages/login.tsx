import { css } from "styled-system/css";
import { Container } from "styled-system/jsx";
import { LoginForm } from "../components/login-form";

export function Login() {
  return (
    <div
      className={css({
        display: "grid",
        placeItems: "center",
        height: "100dvh",
      })}
    >
      <Container w="full" maxW="sm">
        <LoginForm />
      </Container>
    </div>
  );
}
