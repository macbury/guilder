import React, { useMemo, useCallback } from 'react';
import Timeline from 'rsuite/Timeline';
import ReactMarkdown from 'react-markdown'
import dayjs from 'dayjs';
import styled from 'styled-components';
import { Comment } from '../../store/slices/comments';

export interface ICommentProps {
  comment: Comment,
  onDelete(commentId: number);
}

const Header = styled.div`
  opacity: 0.8;
  margin-bottom: 10px;
`;

const Body = styled.div`
  line-height: 22px;
  padding-bottom: 10px;
  margin-bottom: 0px;
  border-bottom: 1px solid var(--rs-gray-600);
`;

export default function Comment({ comment, onDelete } : ICommentProps) {
  const {
    date, id, body
  } = comment;

  const at = useMemo(() => dayjs(date).format('DD MMMM YYYY HH:mm') , [date])

  const handleDelete = useCallback(() => {
    if (confirm('Are you sure?')) {
      onDelete(id)
    }
  }, [id, onDelete])

  return (
    <Timeline.Item>
      <Header>{at} - <a href="#" onClick={handleDelete}>Delete</a></Header>
      <Body>
        <ReactMarkdown children={body} />
      </Body>
    </Timeline.Item>
  )
}
