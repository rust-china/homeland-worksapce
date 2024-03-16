export default function Layout(
  props: Readonly<{
    children: React.ReactNode;
    post: React.ReactNode;
  }>
) {
  return (
    <div>
      {props.post}
      {props.children}
    </div>
  );
}
