import React from 'react';
import Breadcrumb from 'rsuite/Breadcrumb';
import Col from 'rsuite/Col';
import Row from 'rsuite/Row';
import useWindowTitle from '../../components/hooks/useWindowTitle';
import { Container, MarginRow } from '../../components/TablePage';
import {
  BondMonthlyInterest,
  BondMonthlyBalance,
  BondBuyoutPrice,
  BondBuyoutMonthlyPrice,
  BondBalancePrice
} from './BondBalanceChart';

function WidgetCol({ children }) {
  return (
    <Col xs={24} md={12} xl={6} style={{ marginBottom: '15px' }}>
      {children}
    </Col>
  )
}

export default function DashboardPage() {
  useWindowTitle('Dashboard');

  return (
    <Container fluid>
      <MarginRow>
        <Col xs={24}>
          <Breadcrumb>
            <Breadcrumb.Item href="/">Dashboard</Breadcrumb.Item>
          </Breadcrumb>
        </Col>
      </MarginRow>
      <Row>
        <WidgetCol>
          <BondMonthlyInterest />
        </WidgetCol>
        <WidgetCol>
          <BondMonthlyBalance />
        </WidgetCol>
        <WidgetCol>
          <BondBuyoutPrice />
        </WidgetCol>
        <WidgetCol>
          <BondBuyoutMonthlyPrice />
        </WidgetCol>
      </Row>
      <Row>
        <WidgetCol>
          <BondBalancePrice />
        </WidgetCol>
      </Row>
    </Container>
  )
}
