import type { Metadata } from "next";
import { Suspense } from "react";
import Loading from "@/app/loading";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Rust China",
  description: "Rust Learning and Communication Platform",
};

export default function RootLayout(props: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Suspense fallback={<Loading />}>
          {props.children}
        </Suspense>
      </body>
    </html>
  );
}
