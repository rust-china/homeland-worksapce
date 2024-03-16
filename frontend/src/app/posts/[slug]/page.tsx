export default function Page({ params }: { params: { slug: string } }) {
  return <div>post page detail @{params.slug}</div>;
}
