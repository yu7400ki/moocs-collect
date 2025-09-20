import { router } from "@/components/providers/tanstack-router";
import { Button } from "@/components/ui/button";
import { WifiOffIcon } from "lucide-react";
import { css } from "styled-system/css";
import { Container, Flex } from "styled-system/jsx";
import { useAuth } from "../atoms/authenticated";
import { LoginForm } from "../components/login-form";

export function Login() {
  const { goOffline } = useAuth();

  const handleOfflineMode = async () => {
    goOffline();
    await router.navigate({
      to: "/",
    });
  };

  return (
    <div
      className={css({
        display: "grid",
        gridTemplateRows: "token(spacing.16) 1fr token(spacing.16)",
        gridTemplateColumns: "1fr",
        height: "100dvh",
      })}
    >
      <Container
        w="full"
        maxW="md"
        placeSelf="center"
        gridRow="2 / 3"
        gridColumn="1 / 2"
      >
        <div
          className={css({
            display: "grid",
            gap: 4,
          })}
        >
          <LoginForm />
        </div>
      </Container>
      <Flex gridRow="3 / 4" gridColumn="1 / 2" px="4" alignItems="center">
        <Button
          variant="outline"
          onClick={handleOfflineMode}
          size="sm"
          rounded="full"
        >
          <WifiOffIcon />
          オフラインモードで続行
        </Button>
      </Flex>
    </div>
  );
}
