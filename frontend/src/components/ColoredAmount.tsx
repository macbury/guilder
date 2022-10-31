import React from 'react';
import styled from 'styled-components';

const NeutralChange = styled.div`

`;

const PositiveChange = styled(NeutralChange)`
  color: var(--rs-green-500);
`;

const NegativeChange = styled(NeutralChange)`
  color: var(--rs-blue-500);
`;

function componentByAmount(amount : number) {
  if (amount === 0.0) {
    return NeutralChange
  }

  if (amount >= 0.0) {
    return PositiveChange
  }

  return NegativeChange;
}

export interface IColoredAmountProps {
  amount: number,
  children: any
}

export default function ColoredAmount({ amount, children, ...props } : IColoredAmountProps) {
  const Change = componentByAmount(amount);

  return (
    <Change {...props}>
      {children}
    </Change>
  )
}
