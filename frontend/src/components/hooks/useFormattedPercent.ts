import { useMemo } from 'react';

export function formatPercent(percent : number, plusSymbol = false) {
  if (percent == null) {
    return '-'
  }

  const value = (percent || 0.0).toFixed(4);
  if (percent > 0.0 && plusSymbol) {
    return `+${value}%`
  } else {
    return `${value}%`
  }
}

export default function useFormattedPercent(percent : number, plusSymbol = false) {
  return useMemo(() => formatPercent(percent, plusSymbol), [percent, plusSymbol]);
}
