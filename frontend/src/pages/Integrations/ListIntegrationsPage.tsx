import React from 'react';
import Breadcrumb from 'rsuite/Breadcrumb';
import Table from 'rsuite/Table';
import Panel from 'rsuite/Panel';
import Col from 'rsuite/Col';
import { Actions, AddActionButton, Container, ContentRow, FullHeightCol, MarginRow } from '../../components/TablePage';
import ActionsCell from '../../components/table/ActionsCell';
import { useIntegrationsState, useLoadIntegrations } from '../../store/hooks/integrations';
import IntegrationForm from './IntegrationForm';
import DateCell from '../../components/table/DateCell';
import useWindowTitle from '../../components/hooks/useWindowTitle';

export default function ListIntegrationsPage() {
  useWindowTitle('Integrations');
  useLoadIntegrations();

  const {
    state: {
      integrations,
      loading
    },
    actions: {
      newIntegration,
      editIntegration,
      destroy,
      sync
    }
  } = useIntegrationsState();

  return (
    <Container fluid>
      <MarginRow>
        <Col xs={20}>
          <Breadcrumb>
            <Breadcrumb.Item href="/integrations">Integrations</Breadcrumb.Item>
          </Breadcrumb>
        </Col>
        <Actions xs={4}>
          <AddActionButton onClick={newIntegration}>Setup new integration</AddActionButton>
        </Actions>
      </MarginRow>
      <ContentRow>
        <FullHeightCol xs={24}>
          <Panel bordered bodyFill>
            <Table
              fillHeight
              bordered
              cellBordered
              loading={loading}
              data={integrations}>
              <Table.Column fixed flexGrow={1}>
                <Table.HeaderCell>Name</Table.HeaderCell>
                <Table.Cell dataKey="name" />
              </Table.Column>
              <Table.Column width={120}>
                <Table.HeaderCell>Kind</Table.HeaderCell>
                <Table.Cell dataKey="kind" />
              </Table.Column>
              <Table.Column width={120}>
                <Table.HeaderCell>Status</Table.HeaderCell>
                <Table.Cell dataKey="status" />
              </Table.Column>
              <Table.Column width={200}>
                <Table.HeaderCell>Last sync at</Table.HeaderCell>
                <DateCell dataKey="last_sync_at" />
              </Table.Column>
              <Table.Column width={170} align="right">
                <Table.HeaderCell>Actions</Table.HeaderCell>
                <ActionsCell
                  dataKey="id"
                  onSync={sync as any}
                  onEdit={editIntegration as any}
                  onDestroy={destroy as any} />
              </Table.Column>
            </Table>
          </Panel>
        </FullHeightCol>
      </ContentRow>
      <IntegrationForm />
    </Container>
  )
}
