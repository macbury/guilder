import React, { useCallback } from 'react';
import Breadcrumb from 'rsuite/Breadcrumb';
import Table from 'rsuite/Table';
import Panel from 'rsuite/Panel';
import Col from 'rsuite/Col';
import { useNavigate } from 'react-router-dom';

import ActionsCell from '../../components/table/ActionsCell';
import { Actions, AddActionLink, Container, ContentRow, FullHeightCol, MarginRow } from '../../components/TablePage';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { useDestroyWalletMutation, useGetWalletsQuery } from '../../store/api';
import PriceCell from '../../components/table/PriceCell';

export default function ListWalletsPage() {
  useWindowTitle('Wallets');
  const navigate = useNavigate();
  const { data: wallets, isLoading } = useGetWalletsQuery();
  const [destroyWallet] = useDestroyWalletMutation();
  const editCategory = useCallback((walletId: number) => {
    navigate(`/wallets/${walletId}/edit`)
  }, []);

  return (
    <Container fluid>
      <MarginRow>
        <Col xs={20}>
          <Breadcrumb>
            <Breadcrumb.Item href="/wallets">Wallets</Breadcrumb.Item>
          </Breadcrumb>
        </Col>
        <Actions xs={4}>
          <AddActionLink to="/wallets/new">Add new wallet</AddActionLink>
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
              data={wallets}>
              <Table.Column fixed flexGrow={1}>
                <Table.HeaderCell>Name</Table.HeaderCell>
                <Table.Cell dataKey="name" />
              </Table.Column>
              <Table.Column flexGrow={3}>
                <Table.HeaderCell>Description</Table.HeaderCell>
                <Table.Cell dataKey="description" />
              </Table.Column>
              <Table.Column width={140} resizable>
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
                  onEdit={editCategory as any}
                  onDestroy={destroyWallet as any} />
              </Table.Column>
            </Table>
          </Panel>
        </FullHeightCol>
      </ContentRow>
    </Container>
  )
}
