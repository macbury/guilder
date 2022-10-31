import { useCallback } from "react";

export type AbortFunction = () => void
export default function useAbortCallback<T extends (signal : AbortSignal, ...args: any[]) => any>(callback : T, deps : any[]) {
  return useCallback((...args: any[]) => {
    const abortController = new AbortController()
    callback(abortController.signal, ...args);
    return () => abortController.abort()
  }, deps)
}
