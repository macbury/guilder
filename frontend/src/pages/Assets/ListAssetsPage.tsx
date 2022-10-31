import React from 'react';

import Table from 'rsuite/Table';
import Panel from 'rsuite/Panel';
import Dropdown from 'rsuite/Dropdown';
import MenuIcon from '@rsuite/icons/Menu';
import TrashIcon from '@rsuite/icons/Trash';
import TagIcon from '@rsuite/icons/Tag';

import { Gap, ActionsHeader, Container, ContentRow, FullHeightCol, MarginRow, BatchActions, AddActionLink } from '../../components/TablePage';
import Price52ChangeCell from '../../components/table/Price52ChangeCell';
import AssetNameCell from '../../components/table/AssetNameCell';
import PriceCell from '../../components/table/PriceCell';
import PercentChangeCell from '../../components/table/PercentChangeCell';
import CheckCell, { CheckCellHeader } from '../../components/table/CheckCell';
import ActionsCell from '../../components/table/ActionsCell';
import CategoriesFilter from '../../components/Filters/CategoriesFilter';
import CategoryCell from '../../components/table/CategoryCell';
import { useCategoriesParams } from '../../components/hooks/useFilterParams';
import FilterNameInput from '../../components/inputs/FilterNameInput';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { FilterPopup } from '../../components/FilterPopup';
import { useFilterParams } from '../../hooks/useFilterParams';
import { useSortableAssetsState } from '../../store/hooks/assets';
import AssetForm from './AssetForm';

export default function ListAssetsPage() {
  useWindowTitle('Assets');
  const categoriesParams = useCategoriesParams();
  const { options, onSortChange } = useFilterParams();

  const {
    actions: {
      destroy,
      select,
      deselect,
      destroyAll,
      selectAll,
      clearSelection,
      editAssets
    },
    state: {
      selectedCount,
      selectedIds,
      assets,
      loading
    }
  } = useSortableAssetsState(options);

  return (
    <Container fluid>
      <MarginRow>
        <ActionsHeader xs={24}>
          <BatchActions>
            <Dropdown icon={<MenuIcon />} size="lg" title="Batch Actions" disabled={selectedCount == 0}>
              <Dropdown.Item onClick={destroyAll} icon={<TrashIcon />}>Delete selected</Dropdown.Item>
              <Dropdown.Item onClick={editAssets} icon={<TagIcon />}>Set category</Dropdown.Item>
            </Dropdown>
          </BatchActions>

          <FilterNameInput
            param="name"
            placeholder="Search name or ticker" />
          <FilterPopup>
            <CategoriesFilter
              onTickCategory={categoriesParams.setFilterParams}
              selectedCategoryIds={categoriesParams.selectedIds}
            />
          </FilterPopup>
          <Gap/>
          <AddActionLink to="/search">Watch new asset</AddActionLink>
        </ActionsHeader>
      </MarginRow>
      <ContentRow>
        <FullHeightCol xs={24}>
          <Panel bordered bodyFill>
            <Table
              fillHeight
              bordered
              cellBordered
              loading={loading}
              sortColumn={options.sort}
              sortType={options.direction}
              onSortColumn={onSortChange}
              data={assets}>
              <Table.Column width={50} align="center" style={{ padding: 0 }}>
                <Table.HeaderCell>
                  <CheckCellHeader
                    selectAll={selectAll}
                    deselectAll={clearSelection}
                    selectedCount={selectedCount}
                    totalCount={assets.length} />
                </Table.HeaderCell>
                <CheckCell
                  onChange={(checked, id) => checked ? select(id) : deselect(id)}
                  dataKey="id"
                  checkedKeys={selectedIds} />
              </Table.Column>
              <Table.Column fixed flexGrow={1} sortable>
                <Table.HeaderCell>Name</Table.HeaderCell>
                <AssetNameCell dataKey="name" />
              </Table.Column>
              <Table.Column width={200} sortable resizable>
                <Table.HeaderCell>Category</Table.HeaderCell>
                <CategoryCell dataKey="categoryId" />
              </Table.Column>
              <Table.Column width={150} sortable resizable align="right">
                <Table.HeaderCell>Price</Table.HeaderCell>
                <PriceCell dataKey="performance.price" />
              </Table.Column>
              <Table.Column sortable resizable align="right">
                <Table.HeaderCell>D(%)</Table.HeaderCell>
                <PercentChangeCell change dataKey="performance.percentChange" />
              </Table.Column>
              <Table.Column sortable resizable align="right">
                <Table.HeaderCell>YTD(%)</Table.HeaderCell>
                <PercentChangeCell change dataKey="performance.ytdPercentChange" />
              </Table.Column>
              <Table.Column sortable resizable align="right">
                <Table.HeaderCell>Yearly(%)</Table.HeaderCell>
                <PercentChangeCell change dataKey="performance.yearlyPercentChange" />
              </Table.Column>
              <Table.Column width={120} resizable sortable align="right">
                <Table.HeaderCell>52W Range</Table.HeaderCell>
                <Price52ChangeCell dataKey="performance.lowHighScore" />
              </Table.Column>
              <Table.Column width={110}>
                <Table.HeaderCell>Actions</Table.HeaderCell>
                <ActionsCell
                  dataKey="id"
                  onDestroy={destroy} />
              </Table.Column>
            </Table>
          </Panel>
        </FullHeightCol>
      </ContentRow>
      <AssetForm />
    </Container>
  )
}
