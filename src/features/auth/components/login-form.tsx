import { login } from "@/command/login";
import { store } from "@/components/providers/jotai";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { useForm } from "@conform-to/react";
import { parseWithZod } from "@conform-to/zod";
import { useNavigate } from "@tanstack/react-router";
import { useActionState } from "react";
import { css } from "styled-system/css";
import { z } from "zod";
import { authenticatedAtom } from "../atoms/authenticated";

const schema = z.object({
  username: z
    .string({ message: "ユーザー名を入力してください" })
    .nonempty("ユーザー名を入力してください"),
  password: z
    .string({ message: "パスワードを入力してください" })
    .nonempty("パスワードを入力してください"),
});

async function loginAction(
  navigation: ReturnType<typeof useNavigate>,
  _: unknown,
  formData: FormData,
) {
  const submission = parseWithZod(formData, { schema });

  if (submission.status !== "success") {
    return submission.reply();
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
  await navigation({
    to: "/",
  });
}

export function LoginForm() {
  const navigation = useNavigate();
  const [lastResult, action, isPending] = useActionState(
    loginAction.bind(null, navigation),
    undefined,
  );
  const [form, fields] = useForm({
    lastResult,
    onValidate({ formData }) {
      return parseWithZod(formData, { schema });
    },
    shouldValidate: "onSubmit",
    shouldRevalidate: "onInput",
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
          })}
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
          })}
        />
        {fields.password.errors?.map((error) => (
          <Field.ErrorText key={error}>{error}</Field.ErrorText>
        ))}
      </Field.Root>
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
