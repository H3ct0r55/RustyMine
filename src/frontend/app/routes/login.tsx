import { PageShell } from "~/components/PageShell";
import type { Route } from "./+types/login";
import { Field, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "~/components/ui/field";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "~/components/ui/card";
export function meta(_: Route.MetaArgs) {
  return [
    { title: "Login" },
  ];
}

export default function login() {
  return (<PageShell>
    <main className="flex justify-center items-center w-full">
      <Card className="w-96 bg-zinc-900">
        <CardHeader>
          <CardTitle>Login to RustyMine</CardTitle>
          <CardDescription>Enter your username and password to login</CardDescription>
        </CardHeader>
        <CardContent>
          <form>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="username">Username</FieldLabel>
                <Input id="username" placeholder="username" required />
              </Field>
              <Field>
                <FieldLabel htmlFor="password">Password</FieldLabel>
                <Input id="password" placeholder="password" type="password" required />
              </Field>
            </FieldGroup>
          </form>
        </CardContent>
        <CardFooter>
          <Button type="submit" className="w-full">Login</Button>
        </CardFooter>
      </Card>

    </main>
  </PageShell>);
}
