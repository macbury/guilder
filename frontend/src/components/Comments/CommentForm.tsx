import React from 'react';
import Input from 'rsuite/Input';
import Form from 'rsuite/Form';
import ButtonToolbar from 'rsuite/ButtonToolbar';
import Button from 'rsuite/Button';
import { useCommentsState } from '../../store/hooks/comments';

const Textarea = React.forwardRef((props, ref) => <Input {...props} as="textarea" ref={ref} />);

export default function CommentForm(props) {
  const {
    actions: { setBody, create },
    state: { body, errors }
  } = useCommentsState();

  return (
    <Form fluid onSubmit={create}>
      <Form.Group controlId="textarea-1">
        <Form.ControlLabel>Comment:</Form.ControlLabel>
        <Form.Control
          value={body}
          rows={5}
          errorMessage={errors['body']?.join(', ')}
          name="textarea"
          accepter={Textarea}
          onChange={(v) => setBody(v)} />
      </Form.Group>
      <Form.Group>
        <ButtonToolbar>
          <Button appearance="primary" onClick={create}>Add comment</Button>
        </ButtonToolbar>
      </Form.Group>
    </Form>
  );
}
