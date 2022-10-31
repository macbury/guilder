import React, { useMemo } from 'react';
import ColoredAmount from './ColoredAmount';
import useFormattedPrice from './hooks/useFormattedPrice';

export interface IPriceChangeProps {
  price: number
  currency: string
}

export default function PriceChange({ price, currency, ...props } : IPriceChangeProps) {
  const amount = useFormattedPrice(price, currency, true);

  return (
    <ColoredAmount amount={price} {...props}>
      {amount}
    </ColoredAmount>
  )
}
