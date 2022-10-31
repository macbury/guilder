import React from 'react';
import Breadcrumb from 'rsuite/Breadcrumb';
import Table from 'rsuite/Table';
import Panel from 'rsuite/Panel';

import Col from 'rsuite/Col';
import ActionsCell from '../../components/table/ActionsCell';
import { Actions, AddActionButton, Container, ContentRow, FullHeightCol, MarginRow } from '../../components/TablePage';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { useLoadCategories, useCategoriesState } from '../../store/hooks/categories';
import { useGetCategoriesQuery } from '../../store/api';

export default function ListCategoriesPage() {
  useWindowTitle('Categories');

  const { data: categories, isLoading } = useGetCategoriesQuery();

  const {
    actions: {
      newCategory,
      editCategory,
      destroy
    }
  } = useCategoriesState();

  return (
    <Container fluid>
      <MarginRow>
        <Col xs={20}>
          <Breadcrumb>
            <Breadcrumb.Item href="/categories">Categories</Breadcrumb.Item>
          </Breadcrumb>
        </Col>
        <Actions xs={4}>
          <AddActionButton onClick={newCategory}>Add new category</AddActionButton>
        </Actions>
      </MarginRow>
      <ContentRow>
        <FullHeightCol xs={24}>
          <Panel bordered bodyFill>
            <Table
              fillHeight
              bordered
              cellBordered
              loading={isLoading}
              data={categories || []}>
              <Table.Column fixed flexGrow={1}>
                <Table.HeaderCell>Name</Table.HeaderCell>
                <Table.Cell dataKey="name" />
              </Table.Column>
              <Table.Column width={110} align="right">
                <Table.HeaderCell>Actions</Table.HeaderCell>
                <ActionsCell
                  dataKey="id"
                  onEdit={editCategory as any}
                  onDestroy={destroy as any} />
              </Table.Column>
            </Table>
          </Panel>
        </FullHeightCol>
      </ContentRow>
    </Container>
  )
}
