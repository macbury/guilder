import React from 'react';
import styled from 'styled-components';
import { Row } from './Row';
import { CategoryRow } from './CategoryRow';
import useFormattedPercent from '../hooks/useFormattedPercent';
import useFormattedPrice from '../hooks/useFormattedPrice';
import { AccountRow } from './AccountRow';
import { IntegrationRow } from './IntegrationRow';
import { WalletRow } from './WalletRow';

export interface IDetailsProps {
  name: string,
  children: any
}

const Details = styled.div`
  display: flex;
  flex-direction: column;
`;

export interface IDetailsPercentProps {
  name: string,
  children: number
}

export function PercentRow({ name, children, ...props } : IDetailsPercentProps) {
  const percent = useFormattedPercent(children, true);
  return (
    <Row name={name} {...props}>
      {percent}
    </Row>
  )
}

export interface IDetailsPriceProps {
  name: string,
  currency: string
  children: number
}

export function PriceRow({ name, children, currency, ...props } : IDetailsPriceProps) {
  const price = useFormattedPrice(children, currency, false);
  return (
    <Row name={name} {...props}>
      {price}
    </Row>
  )
}

export default Details
Details.Row = Row;
Details.PriceRow = PriceRow;
Details.PercentRow = PercentRow;
Details.CategoryRow = CategoryRow;
Details.AccountRow = AccountRow;
Details.IntegrationRow = IntegrationRow;
Details.WalletRow = WalletRow;
