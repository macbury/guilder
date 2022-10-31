import React from 'react';
import Breadcrumb from 'rsuite/Breadcrumb';
import Table from 'rsuite/Table';
import Panel from 'rsuite/Panel';
import Col from 'rsuite/Col';
import { useAccountsState } from '../../store/hooks/accounts';
import { Actions, AddActionButton, Container, ContentRow, FullHeightCol, MarginRow } from '../../components/TablePage';
import ActionsCell from '../../components/table/ActionsCell';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { useGetAccountsQuery } from '../../store/api';
import PriceCell from '../../components/table/PriceCell';

export default function ListAccountsPage() {
  useWindowTitle('Accounts');
  const { data: accounts, isLoading } = useGetAccountsQuery();

  const {
    actions: {
      newAccount,
      editAccount,
      destroy
    }
  } = useAccountsState();

  return (
    <Container fluid>
      <MarginRow>
        <Col xs={20}>
          <Breadcrumb>
            <Breadcrumb.Item href="/accounts">Accounts</Breadcrumb.Item>
          </Breadcrumb>
        </Col>
        <Actions xs={4}>
          <AddActionButton onClick={newAccount}>Add new account</AddActionButton>
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
              data={accounts || []}>
              <Table.Column fixed flexGrow={1}>
                <Table.HeaderCell>Name</Table.HeaderCell>
                <Table.Cell dataKey="name" />
              </Table.Column>
              <Table.Column width={300} resizable>
                <Table.HeaderCell>Description</Table.HeaderCell>
                <Table.Cell dataKey="description" />
              </Table.Column>
              <Table.Column width={100}>
                <Table.HeaderCell>Currency</Table.HeaderCell>
                <Table.Cell dataKey="currency" />
              </Table.Column>
              <Table.Column width={180} align="right">
                <Table.HeaderCell>Balance</Table.HeaderCell>
                <PriceCell dataKey="balance" />
              </Table.Column>

              <Table.Column width={110} align="right">
                <Table.HeaderCell>Actions</Table.HeaderCell>
                <ActionsCell
                  dataKey="id"
                  onEdit={editAccount as any}
                  onDestroy={destroy as any} />
              </Table.Column>
            </Table>
          </Panel>
        </FullHeightCol>
      </ContentRow>
    </Container>
  )
}
