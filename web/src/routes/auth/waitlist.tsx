import { Waitlist } from '@clerk/clerk-react'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth/waitlist')({
  component: RouteComponent,
})

function RouteComponent() {
  return <Waitlist />
}
