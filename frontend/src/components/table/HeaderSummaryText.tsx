import React from 'react';
import styled from 'styled-components'

const Wrapper = styled.div`
  display: inline-block;
`;

const Amount = styled.div`
  font-size: 18px;
  color: #2eabdf;
  margin-right: -18px;
`;

export interface IHeaderSummaryTextProps {
  children: any
  value: number,
}

export default function HeaderSummaryText({ children, value, ...props } : IHeaderSummaryTextProps) {
  return (
    <Wrapper {...props}>
      <label>{children}</label>
      <Amount>{value}</Amount>
    </Wrapper>
  )
}
