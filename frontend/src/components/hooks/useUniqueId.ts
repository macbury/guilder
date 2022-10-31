import { useMemo } from 'react';

let id = 0;

export default function useUniqueId(prefix : string = "id") {
  return useMemo(() => {
    id += 1;
    return `${prefix}_${id}`
  }, [prefix])
}
