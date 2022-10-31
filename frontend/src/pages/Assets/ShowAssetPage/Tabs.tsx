import { Link, Route, Routes, useMatch, useParams, useResolvedPath } from 'react-router-dom';
import React from 'react';
import Row from 'rsuite/Row';
import Col from 'rsuite/Col';
import Tag from 'rsuite/Tag';
import styled from 'styled-components';

import HomeTab from './HomeTab';
import Comments from '../../../components/Comments';
import { useLoadComments } from '../../../store/hooks/comments';
import { TabsNav } from '../../../components/TabsNav';

const Badge = styled(Tag)`
  margin-left: 10px;
`;

export interface ITabsProps {
  ticker: string
}

export default function Tabs({ ticker } : ITabsProps) {
  const {
    state: {
      comments,
      loading
    }
  } = useLoadComments(ticker, 'Asset');

  return (
    <React.Fragment>
      <Row>
        <Col xs={24}>
          <TabsNav>
            <TabsNav.Item href={`/assets/${ticker}`}>History</TabsNav.Item>
            <TabsNav.Item href={`/assets/${ticker}/holdings`}>Holdings</TabsNav.Item>
            <TabsNav.Item href={`/assets/${ticker}/technical`}>Technical</TabsNav.Item>
            <TabsNav.Item href={`/assets/${ticker}/comments`}>
              Comments
              {!loading && <Badge color="red" size="sm">{comments.length}</Badge>}
            </TabsNav.Item>
          </TabsNav>
        </Col>
      </Row>
      <Row>
        <Routes>
          <Route path="/" element={<HomeTab ticker={ticker} />} />
          <Route path="/comments" element={<Comments />} />
        </Routes>
      </Row>
    </React.Fragment>
  )
}
