import React from 'react';
import styled from 'styled-components';
import useFormattedPercent from '../hooks/useFormattedPercent';
import useFormattedPrice from '../hooks/useFormattedPrice';

export interface IDetailsRowProps {
  name: string,
  children: any
}

const RowContainer = styled.div`
  display: flex;
  flex-direction: row;
`;

const Label = styled.div`
  width: 170px;
  color: var(--rs-text-secondary);
  min-height: 20px;
  line-height: 1.25;
  font-size: 16px;
  padding: 8px 0;
  width: 170px;
  margin-right: 12px;
  text-align: right;
`

const Value = styled.div`
  padding: 8px 0;
`

export function Row({ name, children, ...props } : IDetailsRowProps) {
  return (
    <RowContainer {...props}>
      <Label>{name}</Label>
      <div className="rs-form-control rs-form-control-wrapper">
        <Value>
          {children}
        </Value>
      </div>
    </RowContainer>
  )
}
