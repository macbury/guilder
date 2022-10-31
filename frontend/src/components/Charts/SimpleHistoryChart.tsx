import React, { useCallback, useMemo, useState } from 'react';
import { ResponsiveLine } from '@nivo/line';
import { linearGradientDef } from '@nivo/core'
import get from 'lodash/get';
import dayjs from 'dayjs';
import Loader from 'rsuite/Loader';
import styled from 'styled-components';
import Panel from 'rsuite/Panel';
import theme from '../../themes/nivo.json';

const Container = styled(Panel)`
  height: 240px;
  overflow: visible;

  .rs-panel-body {
    width: 100%;
    height: 100%;

    svg {
      overflow: hidden;
      border-radius: 6px;
    }
  }
`;

const About = styled.div`
  position: absolute;
  top: 0px;
  left: 7px;
  right: 7px;
  font-size: 16px;
  font-weight: 700;
  opacity: 0.65;
  padding: 16px 16px 16px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  .icon {
    float: right;
    color: #fff;
    font-size: 24px;
  }
`;

const State = styled.div`
  align-items: flex-start;
  font-weight: 300;
  justify-content: space-between;
  flex-wrap: nowrap;
  padding: 0px 16px 16px;
  position: absolute;
  top: 45px;

  .value {
    display: inline-block;
    font-size: 2.4em;
    margin-right: 0.25rem;
    line-height: 1.2em;
  }

  .unit {
    flex: 1 1 0%;
    align-self: flex-end;
    display: inline-block;
    font-size: 1.4em;
    font-weight: 400;
    line-height: 1.6em;
    margin-top: 0.1em;
    opacity: 0.6;
    vertical-align: bottom;
  }

  .change {
    margin-left: 10px;
    opacity: 0.6;
  }

  .date {
    display: block;
    font-size: 0.95rem;
    font-weight: 500;
    opacity: 0.75;
    white-space: nowrap;
    animation: 0.15s cubic-bezier(0.215, 0.61, 0.355, 1) 0s 1 normal none running fade;
  }
`

export interface ISimpleHistoryChartProps {
  icon?: any,
  title: string,
  loading: boolean,
  unitKey?: string,
  dateValueKey?: string,
  primaryValueKey: string,
  percentKey?: string,
  color: string,
  data: any[],
  format(value : number)
}

type THoverValue = {
  value: any
  date: any
}

export default function SimpleHistoryChart({ icon, title, color, format, dateValueKey = "date", primaryValueKey, loading, data, unitKey = "currency", ...props }) {
  const formatter = useMemo(() => new Intl.NumberFormat('en-US', { maximumFractionDigits: 2 }), []);
  const [hoverValue, setHoverValue] = useState<THoverValue>(null);
  const unit = get(data, `0.${unitKey}`) || 'unit';
  const chartData = useMemo(() => {
    if (data == null || data.length == 0) {
      return []
    }

    return [
      {
        id: 1,
        color,
        data: data.map((value) => ({
          x: get(value, dateValueKey),
          y: get(value, primaryValueKey)
        }))
      }
    ]
  }, [data, color, primaryValueKey, dateValueKey])

  const todayValue = useMemo(() => {
    const data = get(chartData, '0.data') || [];
    return get(data, `${data.length - 1}.y`) || 0.0
  }, [chartData, primaryValueKey]);

  const onMouseMove = useCallback((point) => {
    setHoverValue({
      value: point.data.y as any,
      date: dayjs(point.data.x).format('YYYY-MM-D')
    })
  }, [setHoverValue])

  if (loading) {
    return (
      <Container {...props}>
        <Loader size="lg" backdrop content="Loading..." vertical />
      </Container>
    )
  }

  return (
    <Container shaded bordered bodyFill {...props}>
      <ResponsiveLine
        theme={theme}
        data={chartData}
        colors={{ datum: "color" }}
        margin={{ top: 60, right: 0, bottom: 0, left: 0 }}
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
        defs={[
          linearGradientDef('gradientA', [
            { offset: 0, color: 'inherit', opacity: 1 },
            { offset: 100, color: 'inherit', opacity: 0.1 },
          ]),
        ]}
        fill={[{ match: '*', id: 'gradientA' }]}
        curve="monotoneX"
        enableGridX={false}
        enableGridY={false}
        enableArea={true}
        enableCrosshair={false}
        yFormat={format}
        axisTop={null}
        axisBottom={null}
        axisLeft={null}
        onMouseMove={onMouseMove}
        onMouseLeave={() => setHoverValue(null)}
        pointSize={10}
        pointColor={{ theme: 'background' }}
        pointBorderWidth={2}
        pointBorderColor={{ from: 'serieColor' }}
        pointLabelYOffset={-12}
        useMesh={true}
      />
      <About>
        <span className="icon">
          {icon}
        </span>
        {title}
      </About>

      <State>
        <span className="value">
          {formatter.format(hoverValue ? hoverValue.value : todayValue)}
        </span>
        <span className="unit">
          {unit}
        </span>
        <span className="date">
          {hoverValue?.date}
        </span>
      </State>
    </Container>
  )
}
