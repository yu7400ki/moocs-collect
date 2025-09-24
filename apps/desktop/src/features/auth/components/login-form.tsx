import { useForm } from "@conform-to/react";
import { parseWithZod } from "@conform-to/zod";
import { memoize } from "es-toolkit/function";
import { use, useActionState, useMemo } from "react";
import { css } from "styled-system/css";
import { z } from "zod";
import { getCredential } from "@/command/get-credential";
import { login } from "@/command/login";
import { store } from "@/components/providers/jotai";
import { router } from "@/components/providers/tanstack-router";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Field } from "@/components/ui/field";
import { authenticatedAtom } from "../atoms/authenticated";

const schema = z.object({
  username: z
    .string({ message: "ユーザー名を入力してください" })
    .nonempty("ユーザー名を入力してください"),
  password: z
    .string({ message: "パスワードを入力してください" })
    .nonempty("パスワードを入力してください"),
  remember: z.boolean().default(false),
});

const memoizedGetCredential = memoize(getCredential, {
  getCacheKey: ({ username }) => username,
});

async function loginAction(_: unknown, formData: FormData) {
  const submission = parseWithZod(formData, { schema });

  if (submission.status !== "success") {
    return submission.reply();
  }

  if (submission.value.remember) {
    window.localStorage.setItem("remember", "true");
    window.localStorage.setItem("username", submission.value.username);
  } else {
    window.localStorage.removeItem("remember");
    window.localStorage.removeItem("username");
  }

  try {
    const loggedIn = await login(submission.value);
    if (!loggedIn) {
      return submission.reply({
        formErrors: ["ユーザー名またはパスワードが違います"],
      });
    }
  } catch (error) {
    return submission.reply({ formErrors: ["エラーが発生しました"] });
  }

  store.set(authenticatedAtom, true);
  await router.navigate({
    to: "/",
  });
}

export function LoginForm() {
  const remember = useMemo(
    () => window.localStorage.getItem("remember") === "true",
    [],
  );
  const username = useMemo(
    () => window.localStorage.getItem("username") ?? "",
    [],
  );
  const password = use(memoizedGetCredential({ username }));
  const [lastResult, action, isPending] = useActionState(
    loginAction,
    undefined,
  );
  const [form, fields] = useForm({
    lastResult,
    onValidate({ formData }) {
      return parseWithZod(formData, { schema });
    },
    shouldValidate: "onSubmit",
    shouldRevalidate: "onInput",
    defaultValue: {
      username,
      password: password ?? "",
      remember,
    },
  });

  return (
    <form
      id={form.id}
      onSubmit={form.onSubmit}
      action={action}
      noValidate
      className={css({ display: "grid", gap: 4 })}
    >
      <Field.Root invalid={!!fields.username.errors}>
        <Field.Label>ユーザー名</Field.Label>
        <Field.Input
          type="text"
          key={fields.username.key}
          defaultValue={fields.username.initialValue}
          name={fields.username.name}
          className={css({
            fontFamily: "latin",
            fontVariantNumeric: "initial",
          })}
          autoComplete="off"
        />
        {fields.username.errors?.map((error) => (
          <Field.ErrorText key={error}>{error}</Field.ErrorText>
        ))}
      </Field.Root>
      <Field.Root invalid={!!fields.password.errors}>
        <Field.Label>パスワード</Field.Label>
        <Field.Input
          type="password"
          key={fields.password.key}
          defaultValue={fields.password.initialValue}
          name={fields.password.name}
          className={css({
            fontFamily: "latin",
            fontVariantNumeric: "initial",
          })}
          autoComplete="off"
        />
        {fields.password.errors?.map((error) => (
          <Field.ErrorText key={error}>{error}</Field.ErrorText>
        ))}
      </Field.Root>
      <Checkbox
        size="sm"
        key={fields.remember.key}
        defaultChecked={fields.remember.initialValue === "on"}
        name={fields.remember.name}
      >
        認証情報を保持する
      </Checkbox>
      <div
        className={css({
          display: "grid",
          gap: 2,
        })}
      >
        {form.errors?.map((error) => (
          <span
            key={error}
            role="alert"
            className={css({
              color: "fg.error",
              fontSize: "sm",
            })}
          >
            {error}
          </span>
        ))}
      </div>
      <Button type="submit" loading={isPending}>
        ログイン
      </Button>
    </form>
  );
}
