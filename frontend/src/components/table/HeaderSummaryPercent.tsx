import React from 'react';
import styled from 'styled-components'
import useFormattedPercent from '../hooks/useFormattedPercent';
import useFormattedPrice from '../hooks/useFormattedPrice';
import PriceChange from '../PriceChange';

const Wrapper = styled.div`
  display: inline-block;
`;

const Amount = styled.div`
  font-size: 18px;
  color: #2eabdf;
  margin-right: -18px;
`;

export interface IHeaderSummaryPercentProps {
  children: any
  value: number,
}

export default function HeaderSummaryPercent({ children, value, ...props } : IHeaderSummaryPercentProps) {
  const percent = useFormattedPercent(value, true);
  return (
    <Wrapper {...props}>
      <label>{children}</label>
      <Amount>{percent}</Amount>
    </Wrapper>
  )
}
