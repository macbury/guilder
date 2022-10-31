import React from 'react';

import Table from 'rsuite/Table';

import Panel from 'rsuite/Panel';
import Dropdown from 'rsuite/Dropdown';
import MenuIcon from '@rsuite/icons/Menu';
import TrashIcon from '@rsuite/icons/Trash';
import TagIcon from '@rsuite/icons/Tag';

import { Container, ContentRow, FullHeightCol, MarginRow, BatchActions, Gap, ActionsHeader } from '../../components/TablePage';
import PriceCell from '../../components/table/PriceCell';
import PercentChangeCell from '../../components/table/PercentChangeCell';
import CheckCell, { CheckCellHeader } from '../../components/table/CheckCell';

import CategoryCell from '../../components/table/CategoryCell';
import { FilterPopup } from '../../components/FilterPopup';
import { useSortableBondsState } from '../../store/hooks/bonds';
import PriceChangeCell from '../../components/table/PriceChangeCell';
import BondsNameCell from '../../components/table/BondNameCell';
import PercentCell from '../../components/table/PercentCell';
import HeaderSummaryPrice from '../../components/table/HeaderSummaryPrice';
import HeaderSummaryPercent from '../../components/table/HeaderSummaryPercent';
import HeaderSummaryPriceChange from '../../components/table/HeaderSummaryPriceChange';
import DaysLeftCell from '../../components/table/DaysLeftCell';
import FilterNameInput from '../../components/inputs/FilterNameInput';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { useCategoriesParams, useAccountsParams, useWalletsParams } from '../../components/hooks/useFilterParams';
import HeaderSummaryText from '../../components/table/HeaderSummaryText';
import CategoriesFilter from '../../components/Filters/CategoriesFilter';
import useBondsFilterParams from './useBondsFilterParams';
import Scopes from '../../components/Scopes';
import AccountsFilter from '../../components/Filters/AccountsFilter';
import BondsForm from './BondsForm';
import AccountCell from '../../components/table/AccountCell';
import WalletCell from '../../components/table/WalletCell';
import WalletsFilter from '../../components/Filters/WalletsFilter';

export default function ListBondsPage() {
  useWindowTitle('Bonds');

  const categoriesParams = useCategoriesParams();
  const accountsParams = useAccountsParams();
  const walletsParams = useWalletsParams();
  const { options, onSortChange } = useBondsFilterParams();

  const {
    actions: {
      clearSelection,
      selectAll,
      select,
      deselect,
      destroyAll,
      showForm
    },
    state: {
      bonds,
      loading,
      summary,
      selectedIds,
      selectedCount
    }
  } = useSortableBondsState(options);

  const edit = () => null
  const destroy = () => null

  return (
    <Container fluid>
      <MarginRow>
        <ActionsHeader xs={24}>
        <BatchActions>
            <Dropdown icon={<MenuIcon />} size="lg" title="Batch Actions" disabled={selectedCount == 0}>
              <Dropdown.Item onClick={destroyAll} icon={<TrashIcon />}>Delete selected</Dropdown.Item>
              <Dropdown.Item onClick={showForm} icon={<TagIcon />}>Mass edit bonds</Dropdown.Item>
            </Dropdown>
          </BatchActions>
          <FilterNameInput
            param="name"
            placeholder="Search name or emission" />
          <FilterPopup>
            <CategoriesFilter
              onTickCategory={categoriesParams.setFilterParams}
              selectedCategoryIds={categoriesParams.selectedIds} />

            <AccountsFilter
              selectedAccountsIds={accountsParams.selectedIds}
              onTickAccount={accountsParams.setFilterParams} />

            <WalletsFilter
              selectedWalletIds={walletsParams.selectedIds}
              onTickWallet={walletsParams.setFilterParams}
            />
          </FilterPopup>
          <Gap/>
          <Scopes
            defaultScope="active"
            options={[{ key: 'active', name: 'Active' }, { key: 'archived', name: 'Archived' }, { key: 'all', name: 'All' }]} />
        </ActionsHeader>
      </MarginRow>
      <ContentRow>
        <FullHeightCol xs={24}>
          <Panel bordered bodyFill>
            <Table
              headerHeight={70}
              fillHeight
              affixHeader
              affixHorizontalScrollbar
              bordered
              cellBordered
              loading={loading}
              sortColumn={options.sort}
              sortType={options.direction}
              onSortColumn={onSortChange}
              data={bonds}>
              <Table.Column width={50} align="center" style={{ padding: 0 }}>
                <Table.HeaderCell>
                  <CheckCellHeader
                    selectAll={selectAll}
                    deselectAll={clearSelection}
                    selectedCount={selectedCount}
                    totalCount={bonds.length} />
                </Table.HeaderCell>
                <CheckCell
                  onChange={(checked, id) => checked ? select(id) : deselect(id)}
                  dataKey="id"
                  checkedKeys={selectedIds} />
              </Table.Column>
              <Table.Column minWidth={100} flexGrow={1} sortable>
                <Table.HeaderCell>Emission</Table.HeaderCell>
                <BondsNameCell dataKey="emission" />
              </Table.Column>
              <Table.Column width={150} sortable resizable>
                <Table.HeaderCell>Category</Table.HeaderCell>
                <CategoryCell dataKey="categoryId" />
              </Table.Column>
              <Table.Column width={150} sortable resizable>
                <Table.HeaderCell>Account</Table.HeaderCell>
                <AccountCell dataKey="accountId" />
              </Table.Column>
              <Table.Column width={150} sortable resizable>
                <Table.HeaderCell>Wallet</Table.HeaderCell>
                <WalletCell dataKey="walletId" />
              </Table.Column>
              <Table.Column sortable resizable align="right">
                <Table.HeaderCell>
                  <HeaderSummaryText value={summary.shares}>Shares</HeaderSummaryText>
                </Table.HeaderCell>
                <Table.Cell dataKey="performance.shares" />
              </Table.Column>
              <Table.Column sortable resizable align="right" width={120}>
                <Table.HeaderCell>Start date</Table.HeaderCell>
                <Table.Cell dataKey="startDate" />
              </Table.Column>
              <Table.Column sortable resizable align="right" width={120}>
                <Table.HeaderCell>
                  <HeaderSummaryPercent value={summary.avgRate}>Rate</HeaderSummaryPercent>
                </Table.HeaderCell>
                <PercentCell dataKey="performance.currentRate" />
              </Table.Column>
              <Table.Column sortable resizable align="right">
                <Table.HeaderCell>Interest in</Table.HeaderCell>
                <DaysLeftCell
                  tooltipKey="interestDate"
                  dataKey="performance.interestDaysLeft" />
              </Table.Column>
              <Table.Column sortable width={120} resizable align="right">
                <Table.HeaderCell>Buyout</Table.HeaderCell>
                <Table.Cell dataKey="endDate" />
              </Table.Column>
              <Table.Column width={170} sortable resizable align="right">
                <Table.HeaderCell>
                  <HeaderSummaryPrice currency="PLN" amount={summary.startPrice}>Start Value</HeaderSummaryPrice>
                </Table.HeaderCell>
                <PriceCell dataKey="performance.startPrice" />
              </Table.Column>
              <Table.Column sortable resizable align="right" width={140}>
                <Table.HeaderCell>
                  <HeaderSummaryPriceChange currency="PLN" amount={summary.dayPriceChange}>Today</HeaderSummaryPriceChange>
                </Table.HeaderCell>
                <PriceChangeCell change dataKey="performance.dayPriceChange" />
              </Table.Column>
              <Table.Column sortable resizable align="right">
                <Table.HeaderCell>Today(%)</Table.HeaderCell>
                <PercentChangeCell change dataKey="performance.dayPercentChange" />
              </Table.Column>
              <Table.Column width={170} sortable resizable align="right">
                <Table.HeaderCell>
                  <HeaderSummaryPrice currency="PLN" amount={summary.currentPrice}>Current Value</HeaderSummaryPrice>
                </Table.HeaderCell>
                <PriceCell dataKey="performance.price" />
              </Table.Column>
              <Table.Column width={170} sortable resizable align="right">
                <Table.HeaderCell>
                  <HeaderSummaryPrice currency="PLN" amount={summary.buyoutPrice}>Buyout Value</HeaderSummaryPrice>
                </Table.HeaderCell>
                <PriceCell dataKey="performance.buyoutPrice" />
              </Table.Column>
              <Table.Column sortable resizable align="right">
                <Table.HeaderCell>CP(%)</Table.HeaderCell>
                <PercentChangeCell change dataKey="performance.percentChange" />
              </Table.Column>
              <Table.Column sortable resizable align="right" width={120}>
                <Table.HeaderCell>Updated at</Table.HeaderCell>
                <Table.Cell dataKey="performance.priceDate" />
              </Table.Column>
            </Table>
          </Panel>
        </FullHeightCol>
      </ContentRow>
      <BondsForm />
    </Container>
  )
}
