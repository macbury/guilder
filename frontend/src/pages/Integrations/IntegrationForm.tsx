import React from 'react'
import Drawer from 'rsuite/Drawer';
import Button from 'rsuite/Button';
import Form from 'rsuite/Form';
import { useIntegrationsState } from '../../store/hooks/integrations';
import IntegrationKindSelect from '../../components/inputs/IntegrationKindSelect';

export default function IntegrationForm() {
  const {
    state: {
      loading,
      form: {
        integrationId,
        visible,
        value,
        errors
      }
    },

    actions: {
      closeForm,
      updateForm,
      save,
      update
    }
  } = useIntegrationsState();

  const existingRecord = !!integrationId;
  const submit = existingRecord ? update : save;

  return (
    <Drawer open={visible} onClose={closeForm} backdrop="static">
      <Drawer.Header>
        <Drawer.Title>{existingRecord ? 'Edit Integration' : 'New integration'}</Drawer.Title>
        <Drawer.Actions>
          <Button onClick={closeForm}>Cancel</Button>
          <Button onClick={submit} appearance="primary" loading={loading}>
            {existingRecord ? 'Update' : 'Save'}
          </Button>
        </Drawer.Actions>
      </Drawer.Header>
      <Drawer.Body>
        <Form
          onSubmit={submit}
          disabled={loading}
          fluid
          formValue={value}
          onChange={updateForm}
        >
          <Form.Group controlId="kind">
            <Form.ControlLabel>Integration:</Form.ControlLabel>
            <Form.Control
              size="lg"
              block
              disabled={existingRecord}
              accepter={IntegrationKindSelect}
              errorMessage={errors['kind']?.join(', ')}
              name="kind" />
          </Form.Group>

          <Form.Group controlId="name">
            <Form.ControlLabel>Name:</Form.ControlLabel>
            <Form.Control
              errorMessage={errors['name']?.join(', ')}
              name="name" />
          </Form.Group>

          <Form.Group controlId="login">
            <Form.ControlLabel>Login:</Form.ControlLabel>
            <Form.Control
              errorMessage={errors['login']?.join(', ')}
              name="login" />
          </Form.Group>

          <Form.Group controlId="password">
            <Form.ControlLabel>Password:</Form.ControlLabel>
            <Form.Control
              type="password"
              errorMessage={errors['password']?.join(', ')}
              name="password" />
          </Form.Group>
        </Form>
      </Drawer.Body>
    </Drawer>
  )
}
