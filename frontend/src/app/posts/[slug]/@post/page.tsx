export default function Page({ params }: { params: { slug: string } }) {
  return <div>post content page @{params.slug}</div>;
}
