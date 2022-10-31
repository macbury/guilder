import { useParams } from 'react-router-dom';
import React, { useEffect } from 'react';
import styled from 'styled-components';

import Grid from 'rsuite/Grid';
import Row from 'rsuite/Row';
import Col from 'rsuite/Col';
import Loader from 'rsuite/Loader';
import Tabs from './Tabs';
import Breadcrumb, { BreadcrumbItem } from '../../../components/Breadcrump';
import { useAssetsState } from '../../../store/hooks/assets';
import useWindowTitle from '../../../components/hooks/useWindowTitle';
import { AssetHeader } from '../../../components/AssetHeader';

const Container = styled.div`
  margin-bottom: 20px;
`;

const MarginRow = styled(Row)`
  margin-bottom: 20px;
`;

export default function ShowAssetPage() {
  const { ticker } = useParams();
  const {
    state: { asset, loading },
    actions: { fetchAsset }
  } = useAssetsState();

  useWindowTitle(asset ? asset.name : 'Show asset');

  useEffect(() => {
    fetchAsset(ticker);
  }, [ticker])

  if (loading || !asset) {
    return <Loader size="lg" inverse center content="loading..." />
  }

  return (
    <Container>
      <Grid fluid>
        <Row>
          <Col xs={24}>
            <Breadcrumb>
              <BreadcrumbItem href="/assets">Assets</BreadcrumbItem>
              <Breadcrumb.Item active>{ticker}</Breadcrumb.Item>
            </Breadcrumb>
          </Col>
        </Row>
        <MarginRow>
          <Col xs={24}>
            <AssetHeader
              secondaryLogoUrl={asset.secondaryLogoUrl}
              logoUrl={asset.logoUrl}
              currency={asset.currency}
              name={asset.name}
              description={asset.description}
              price={asset?.performance?.price}
              priceChange={asset?.performance?.priceChange}
              percentChange={asset?.performance?.percentChange}
            />
          </Col>
        </MarginRow>

        <Tabs ticker={ticker} />
      </Grid>
    </Container>
  )
}
