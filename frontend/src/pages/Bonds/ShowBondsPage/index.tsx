import React from 'react';
import styled from 'styled-components';
import { Route, Routes, useParams } from 'react-router-dom';
import Loader from 'rsuite/Loader';
import Grid from 'rsuite/Grid';
import Row from 'rsuite/Row';
import Col from 'rsuite/Col';
import Tag from 'rsuite/Tag';

import Breadcrumb, { BreadcrumbItem } from '../../../components/Breadcrump';
import useWindowTitle from '../../../components/hooks/useWindowTitle';
import { useBondState } from '../../../store/hooks/bonds';
import { AssetHeader } from '../../../components/AssetHeader';
import DetailsTab from './DetailsTab';
import { TabsNav } from '../../../components/TabsNav';
import { useLoadComments } from '../../../store/hooks/comments';
import Comments from '../../../components/Comments';
import InterestTableTab from './InterestTableTab';
import BondPerformanceTab from './BondPerformanceTab';

export const ContentRow = styled(Row)`
  flex: 1;
`;

const Container = styled.div`
  margin-bottom: 20px;
  flex: 1;
  height: 100%;
`;

const FullGrid = styled(Grid)`
  height: 100%;
`;

const MarginRow = styled(Row)`
  margin-bottom: 20px;
`;

const Badge = styled(Tag)`
  margin-left: 10px;
`;

export default function ShowBondsPage() {
  const { id } = useParams<any>();
  const {
    state: {
      loading,
      bond
    }
  } = useBondState(id as any);

  const {
    state: {
      comments
    }
  } = useLoadComments(id, 'Bond');

  useWindowTitle(bond ? bond.emission : 'Show bond');

  if (loading || !bond) {
    return <Loader size="lg" inverse center content="loading..." />
  }

  const performance : any = bond.performance || {};

  return (
    <Container>
      <FullGrid fluid>
        <Row>
          <Col xs={24}>
            <Breadcrumb>
              <BreadcrumbItem href="/bonds">Bonds</BreadcrumbItem>
              <Breadcrumb.Item active>{bond.emission}</Breadcrumb.Item>
            </Breadcrumb>
          </Col>
        </Row>
        <MarginRow>
          <Col xs={24}>
            <AssetHeader
              currency={bond.currency}
              name={bond.name}
              price={performance.price}
              priceChange={performance.dayPriceChange}
              percentChange={performance.dayPercentChange}
            />
          </Col>
        </MarginRow>

        <MarginRow>
          <Col xs={24}>
            <TabsNav>
              <TabsNav.Item href={`/bonds/${bond.id}`}>Details</TabsNav.Item>
              <TabsNav.Item href={`/bonds/${bond.id}/performance`}>Performance</TabsNav.Item>
              <TabsNav.Item href={`/bonds/${bond.id}/interest_table`}>Interest Table</TabsNav.Item>
              <TabsNav.Item href={`/bonds/${bond.id}/comments`}>
                Comments
                {!loading && <Badge color="red" size="sm">{comments.length}</Badge>}
              </TabsNav.Item>
            </TabsNav>
          </Col>
        </MarginRow>

        <ContentRow>
          <Routes>
            <Route path="/" element={<DetailsTab bond={bond} />} />
            <Route path="/performance" element={<BondPerformanceTab currency={bond.currency} bondId={bond.id}  />} />
            <Route path="/interest_table" element={<InterestTableTab bondId={bond.id} />} />
            <Route path="/comments" element={<Comments />} />
          </Routes>
        </ContentRow>
      </FullGrid>
    </Container>
  )
}
