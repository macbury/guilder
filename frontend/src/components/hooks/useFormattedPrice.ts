import { useMemo, useCallback } from 'react';

export function usePriceFormatter(currency : string, asCurrency: boolean = true) {
  const formatter = useMemo(() => {
    return new Intl.NumberFormat('en-US', {
      style: asCurrency ? 'currency' : undefined,
      currency: currency || 'USD'
    })
  }, [currency])

  return useCallback((price : number) => {
    return formatter.format(price || 0)
  }, [formatter]);
}

export default function useFormattedPrice(price : number, currency : string, plusSymbol = false) {
  const format = usePriceFormatter(currency);
  return useMemo(() => {
    if (!price) {
      return '-'
    }
    const text = format(price || 0);
    if (price > 0.0 && plusSymbol) {
      return `+${text}`
    } else {
      return text
    }
  }, [price, format, plusSymbol]);
}
