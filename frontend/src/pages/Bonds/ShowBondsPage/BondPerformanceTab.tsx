import React, { useMemo } from 'react';
import Col from 'rsuite/Col';
import get from 'lodash/get'
import PanelGroup from 'rsuite/PanelGroup';
import Panel from 'rsuite/Panel';
import MonthlyMoneyLineChart from '../../../components/Charts/MonthlyMoneyLineChart';
import { useGetBondPerformanceQuery, BondMonthlyPerformance } from '../../../store/api';
import MonthlyPercentLineChart from '../../../components/Charts/MonthlyPercentLineChart';

function useChartData(performance: BondMonthlyPerformance[], key : string) {
  return useMemo(() => {
    if (performance.length == 0) {
      return []
    }

    return [{
      id: "data",
      data: performance.map((v) => ({
        x: new Date(v.date),
        y: get(v, key)
      }))
    }]
  }, [performance, key])
}

export default function BondPerformanceTab({ bondId, currency }) {
  const { data, isLoading } = useGetBondPerformanceQuery(bondId);
  const performance = data?.performance || [];
  const priceChangeData = useChartData(performance, 'priceChange');
  const percentChangeData = useChartData(performance, 'percentChange');

  return (
    <Col xs={24}>
      <PanelGroup bordered>
        <Panel header={`Month(${currency})`}>
          <MonthlyMoneyLineChart
            currency={currency}
            data={priceChangeData}
            loading={isLoading}
          />
        </Panel>
        <Panel header="Month(%)">
          <MonthlyPercentLineChart
            data={percentChangeData}
            loading={isLoading}
          />
        </Panel>
      </PanelGroup>
    </Col>
  )
}
