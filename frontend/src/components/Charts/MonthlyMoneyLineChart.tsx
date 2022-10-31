import React from 'react';
import styled from 'styled-components';
import { ResponsiveLine } from '@nivo/line';
import Loader from 'rsuite/Loader';
import theme from '../../themes/nivo.json';
import { usePriceFormatter } from '../hooks/useFormattedPrice';

const Container = styled.div`
  height: 600px;
`;

export interface IMonthlyMoneyLineChartProps {
  currency: string,
  data: any,
  loading: boolean
}

export default function MonthlyMoneyLineChart({ data, currency, loading, ...props } : IMonthlyMoneyLineChartProps) {
  const formatPrice = usePriceFormatter(currency);

  if (loading) {
    return <Loader size="lg" backdrop content="Loading..." vertical {...props} />
  }

  return (
    <Container>
      <ResponsiveLine
        theme={theme}
        data={data}
        margin={{ top: 50, right: 50, bottom: 60, left: 130 }}
        xScale={{
          type: 'time',
          format: '%Y-%m-%d',
          useUTC: false,
          precision: 'day',
        }}
        xFormat="time:%Y-%m-%d"
        yScale={{
          type: 'linear',
          min: 'auto',
          max: 'auto',
          stacked: true,
          reverse: false
        }}
        yFormat={formatPrice}
        axisTop={null}
        axisRight={null}
        axisBottom={{
          format: '%B %Y',
          orient: 'bottom',
          tickSize: 5,
          tickPadding: 5,
          tickRotation: -20,
          tickValues: 'every 1 month',
          legendOffset: 36,
          legendPosition: 'middle'
        }}
        axisLeft={{
          orient: 'left',
          tickSize: 5,
          tickPadding: 5,
          format: formatPrice,
          legendOffset: -10,
          legendPosition: 'middle'
        }}
        pointSize={10}
        colors={{ scheme: 'dark2' }}
        pointColor={{ theme: 'background' }}
        pointBorderWidth={2}
        pointBorderColor={{ from: 'serieColor' }}
        pointLabelYOffset={-12}
        useMesh={true}
        {...props}
      />
    </Container>
  )
}
