import { ResponsiveFunnel } from '@nivo/funnel';
import React from 'react'
import Col from 'rsuite/Col'

import Details from '../../../components/Details'
import { formatPercent } from '../../../components/hooks/useFormattedPercent';
import { Bond } from '../../../store/slices/bonds';
import theme from '../../../themes/nivo.json';

export interface IDetailsTab {
  bond: Bond
}

export default function DetailsTab({ bond } : IDetailsTab) {
  const performance : any = bond.performance || {};
  const data = (performance.rates || []).map((rate) => ({
    id: rate,
    value: rate,
    label: formatPercent(rate)
  }))

  return (
    <React.Fragment>
      <Col xs={13} xsOffset={1}>
        <Details>
          <Details.CategoryRow categoryId={bond.walletId} name="Wallet" />
          <Details.CategoryRow categoryId={bond.categoryId} name="Category" />
          <Details.AccountRow accountId={bond.accountId} name="Account" />
          <Details.IntegrationRow integrationId={bond.integrationId} name="Integration" />
          <Details.Row name="Status">{bond.status}</Details.Row>
          <Details.Row name="Kind">{bond.kind}</Details.Row>
          <Details.Row name="Emission">{bond.emission}</Details.Row>
          <Details.PriceRow name="Start price" currency={bond.currency}>{performance.startPrice}</Details.PriceRow>
          <Details.PriceRow name="Buyout price" currency={bond.currency}>{performance.buyoutPrice}</Details.PriceRow>
          <Details.Row name="Shares">{performance.shares}</Details.Row>
          <Details.PercentRow name="Rate">{performance.currentRate}</Details.PercentRow>
          <Details.Row name="Start day">{bond.startDate}</Details.Row>
          <Details.Row name="Interest day">{bond.interestDate}</Details.Row>
          <Details.Row name="Buyout day">{bond.endDate}</Details.Row>
          <Details.Row name="Buyout Days Left">{performance.buyoutDaysLeft} days</Details.Row>
          <Details.Row name="Interest Days Left">{performance.interestDaysLeft} days</Details.Row>
          <Details.PriceRow currency={bond.currency} name="Price change">{performance.priceChange}</Details.PriceRow>
          <Details.PercentRow name="Price change(%)">{performance.percentChange}</Details.PercentRow>
        </Details>
      </Col>
      <Col xs={10} style={{ height: '600px' }}>
        Rates
        <ResponsiveFunnel
          data={data}
          margin={{ top: 20, right: 20, bottom: 20, left: 20 }}
          shapeBlending={0.33}
          valueFormat=" >-.3s"
          colors={{ scheme: 'nivo' }}
          borderWidth={17}
          theme={theme}
          beforeSeparatorLength={100}
          beforeSeparatorOffset={20}
          afterSeparatorLength={100}
          afterSeparatorOffset={20}
          currentPartSizeExtension={10}
          currentBorderWidth={40}
          motionConfig="wobbly"
      />
      </Col>
    </React.Fragment>
  )
}
