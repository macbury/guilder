import React, { useEffect } from 'react';
import styled from 'styled-components';
import Timeline from 'rsuite/Timeline';
import Loader from 'rsuite/Loader';
import Col from 'rsuite/Col';
import Comment from './Comment';
import CommentForm from './CommentForm';
import { useCommentsState } from '../../store/hooks/comments';

const Container = styled.div`
  padding: 25px 20px;
`;

export default function Comments() {
  const {
    state: {
      loading,
      comments
    },
    actions: {
      destroy
    }
  } = useCommentsState();

  const items = comments.map((comment) => {
    return (
      <Comment
        onDelete={destroy}
        comment={comment}
        key={comment.id} />
    )
  });

  return (
    <Col xs={24}>
      <Container>
        <CommentForm />
      </Container>
      <Container>
        <Timeline>
          {items}
          {loading && <Loader backdrop content="loading..." vertical /> }
        </Timeline>
      </Container>
  </Col>
  )
}
