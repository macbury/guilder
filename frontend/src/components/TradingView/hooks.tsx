import { useLayoutEffect, useRef } from 'react'

export function useScript(src : string, data : any) {
  const viewRef = useRef<HTMLDivElement>()

  useLayoutEffect(() => {
    const script = document.createElement('script');
    script.src = src;
    script.async = false;
    script.innerHTML = JSON.stringify(data)
    viewRef.current?.appendChild(script);

    return () => {
      if (viewRef.current) {
        viewRef.current.innerHTML = ''
      }
    }
  }, [src, data?.symbol])

  return viewRef
}
