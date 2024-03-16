"use client"
export default function Error({ error, reset }: { error: Error, reset: () => void }) {
	return (
		<div>
			页面出错了
			<button onClick={() => reset()}>重试一下</button>
		</div>
	)
}