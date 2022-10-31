import React from 'react';
import styled from 'styled-components'
import useFormattedPrice from '../hooks/useFormattedPrice';
import PriceChange from '../PriceChange';

const Wrapper = styled.div`
  display: inline-block;
`;

const Amount = styled.div`
  font-size: 18px;
  margin-right: -18px;
`;

export interface IHeaderSummaryPriceChangeProps {
  children: any
  amount: number,
  currency: string
}

export default function HeaderSummaryPriceChange({ children, amount, currency, ...props } : IHeaderSummaryPriceChangeProps) {
  return (
    <Wrapper {...props}>
      <label>{children}</label>
      <Amount>
        <PriceChange price={amount} currency={currency} />
      </Amount>
    </Wrapper>
  )
}
