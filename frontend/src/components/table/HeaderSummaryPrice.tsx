import React from 'react';
import styled from 'styled-components'
import useFormattedPrice from '../hooks/useFormattedPrice';

const Wrapper = styled.div`
  display: inline-block;
`;

const Amount = styled.div`
  font-size: 18px;
  color: #2eabdf;
  margin-right: -18px;
`;

export interface IHeaderSummaryPriceProps {
  children: any
  amount: number,
  currency: string
}

export default function HeaderSummaryPrice({ children, amount, currency, ...props } : IHeaderSummaryPriceProps) {
  let value = useFormattedPrice(amount, currency, false);
  return (
    <Wrapper {...props}>
      <label>{children}</label>
      <Amount>{value}</Amount>
    </Wrapper>
  )
}
