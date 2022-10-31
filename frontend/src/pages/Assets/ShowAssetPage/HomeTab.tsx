import React from 'react';
import Col from 'rsuite/Col';
import Fundamentals from '../../../components/TradingView/Fundamentals';
import TickerChart from '../../../components/TradingView/TickerChart';

export interface IHomeTabProps {
  ticker: string
}

export default function HomeTab({ ticker } : IHomeTabProps) {
  return (
    <React.Fragment>
      <Col xs={19}>
        <TickerChart ticker={ticker} />
      </Col>
      <Col xs={5}>
        <Fundamentals ticker={ticker} />
      </Col>
    </React.Fragment>
  )
}
