import React, { useMemo } from 'react'
import Table from 'rsuite/Table';
import groupBy from 'lodash/groupBy';
import PercentCell from '../../../components/table/PercentCell';
import PercentChangeCell from '../../../components/table/PercentChangeCell';
import PriceCell from '../../../components/table/PriceCell';
import PriceChangeCell from '../../../components/table/PriceChangeCell';
import { FullHeightCol } from '../../../components/TablePage';
import { BondInterestRateHistory, useGetInterestRateHistoryQuery } from '../../../store/api';

function useInterestTree(historyData : BondInterestRateHistory) {
  return useMemo(() => {
    if (historyData == null) {
      return []
    }

    const { history, periods } = historyData;
    const groups = groupBy(history, 'period');
    console.log('periods', periods);
    console.log('groups', groups);
    return periods.map((period, index) => {
      return {
        id: index,
        startDate: period.startDate,
        date: period.endDate,
        rate: period.rate,
        children: (groups[index] || []).map((item, itemIndex) => ({
          ...item,
          id: `${index}-${itemIndex}`
        }))
      }
    })
  }, [historyData])
}

export default function InterestTableTab({ bondId }) {
  const { data, error, isLoading } = useGetInterestRateHistoryQuery(bondId);
  const items = useInterestTree(data);

  return (
    <React.Fragment>
      <FullHeightCol xs={24}>
        <Table
          loading={isLoading}
          bordered
          isTree
          rowKey="id"
          cellBordered
          height={800}
          data={items}>
          <Table.Column flexGrow={1} align="right" fixed>
            <Table.HeaderCell>Start Date</Table.HeaderCell>
            <Table.Cell dataKey="startDate" />
          </Table.Column>
          <Table.Column flexGrow={1} align="right" fixed>
            <Table.HeaderCell>Date</Table.HeaderCell>
            <Table.Cell dataKey="date" />
          </Table.Column>
          <Table.Column align="right" width={120}>
            <Table.HeaderCell>Rate </Table.HeaderCell>
            <PercentCell dataKey="rate" />
          </Table.Column>
          <Table.Column align="right" width={200}>
            <Table.HeaderCell>Amount</Table.HeaderCell>
            <PriceCell dataKey="price" />
          </Table.Column>
          <Table.Column align="right" width={140}>
            <Table.HeaderCell>Day change</Table.HeaderCell>
            <PriceChangeCell change dataKey="dayPriceChange" />
          </Table.Column>
          <Table.Column width={180} align="right">
            <Table.HeaderCell>Day change(%)</Table.HeaderCell>
            <PercentChangeCell change dataKey="dayPercentChange" />
          </Table.Column>
          <Table.Column align="right" width={200}>
            <Table.HeaderCell>Total Return</Table.HeaderCell>
            <PriceCell dataKey="priceChange" />
          </Table.Column>
          <Table.Column width={180} align="right">
            <Table.HeaderCell>Total Return(%)</Table.HeaderCell>
            <PercentChangeCell change dataKey="percentChange" />
          </Table.Column>
        </Table>
      </FullHeightCol>
    </React.Fragment>

  )
}
