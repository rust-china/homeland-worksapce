export default function Layout(
  props: Readonly<{
    children: React.ReactNode;
    posts: React.ReactNode;
  }>
) {
  return (
    <div>
      {props.children}
      {props.posts}
    </div>
  );
}
