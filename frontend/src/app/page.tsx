import { redirect } from "next/navigation";
import Link from "next/link";

export default function Home() {
  redirect("/posts");
  // return (
  //   <main className="flex min-h-screen">
  //     homepage
  //     <Link href="/auth/login">登录</Link>
  //   </main>
  // );
}
