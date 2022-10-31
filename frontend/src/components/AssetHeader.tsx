import React, { useState } from 'react';
import styled from 'styled-components';
import truncate from 'lodash/truncate';
import Avatar from 'rsuite/Avatar';
import AvatarGroup from 'rsuite/AvatarGroup';
import useFormattedPrice from './hooks/useFormattedPrice';
import useFormattedPercent from './hooks/useFormattedPercent';
import ColoredAmount from './ColoredAmount';

const Header = styled.div`
  display: flex;
  flex-direction: row;
`;

const HeaderLogo = styled.div`
  width: 150px;
  justify-content: center;
  display: flex;
  flex-direction: column;
  align-items: center;
`

const HeaderAbout = styled.div`
  flex: 1;
`;

const HeaderPrice = styled.div`
  width: 200px;
  text-align: right;
  display: flex;
  flex-direction: column;
  justify-content: center;
  font-size: 18px;
`;

function Description({ text }) {
  const [expanded, setExpanded] = useState(false);
  const desc = expanded ? text : truncate(text, { length: 400 });
  return (
    <React.Fragment>
      {desc}
      {!expanded && <a href="#" onClick={() => setExpanded(true)}>more</a>}
    </React.Fragment>
  );
}

export interface IAssetHeaderProps {
  secondaryLogoUrl?: string,
  logoUrl?: string,
  currency: string,
  name: string,
  description?: string,
  price: number,
  priceChange: number,
  percentChange: number
}

export function AssetHeader(props : IAssetHeaderProps) {
  const {
    secondaryLogoUrl,
    logoUrl,
    name,
    description,
    currency
  } = props;

  const price = useFormattedPrice(props.price, currency, false);
  const priceChange = useFormattedPrice(props.priceChange, currency, true);
  const percentChange = useFormattedPercent(props.percentChange, true);
  const withLogo = secondaryLogoUrl || logoUrl;

  return (
    <Header>
      {withLogo && <HeaderLogo>
        <AvatarGroup stack>
          {secondaryLogoUrl && <Avatar
            size="lg"
            src={secondaryLogoUrl}
            circle />}
          {logoUrl && <Avatar
            size="lg"
            src={logoUrl}
            circle />}
        </AvatarGroup>
      </HeaderLogo>}

      <HeaderAbout>
        <h3>{name}</h3>
        {
          description && <p>
            <Description text={description} />
          </p>
        }
      </HeaderAbout>

      <HeaderPrice>
        <h3>{price}</h3>
        <ColoredAmount amount={props.priceChange}>
          {priceChange} ({percentChange})
        </ColoredAmount>
      </HeaderPrice>
    </Header>
  )
}
