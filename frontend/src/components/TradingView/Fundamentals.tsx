import React, { useMemo } from 'react';
import styled from "styled-components";
import { useScript } from './hooks';

const Inner = styled.div`
  height: 800px;
`;

export default function Fundamentals({ ticker, ...props }) {
  const data = useMemo(() => {
    return {
      "symbol": ticker,
      "colorTheme": "dark",
      "isTransparent": false,
      "largeChartUrl": "",
      "displayMode": "regular",
      "width": "100%",
      "height": "100%",
    }
  }, [ticker])
  const scriptRef = useScript("https://s3.tradingview.com/external-embedding/embed-widget-financials.js", data);

  return (
    <Inner ref={scriptRef} {...props}>
      <div className="tradingview-widget-container__widget"></div>
    </Inner>
  );
}
