import React, { useLayoutEffect } from 'react';
import styled from 'styled-components';
import useUniqueId from '../hooks/useUniqueId';

const Inner = styled.div`
  height: 800px;
`;

const Container = styled.div`

`;

export default function TickerChart({ ticker, ...props }) {
  const domId = useUniqueId();
  useLayoutEffect(() => {
    new TradingView.widget(
      {
        "autosize": true,
        "symbol": ticker,
        "interval": "D",
        "timezone": "Etc/UTC",
        "theme": "dark",
        "style": "1",
        "toolbar_bg": "#f1f3f6",
        "enable_publishing": false,
        "allow_symbol_change": true,
        "container_id": domId,
        "withdateranges": true,
      }
    )
  }, [domId, ticker]);

  return (
    <Container className="tradingview-widget-container" {...props}>
      <Inner id={domId} />
    </Container>
  )
}
