import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { PublicPortfolioView } from "../../../components/PublicPortfolioView";
import { fetchPublicUser } from "../../../lib/api";

type Props = { params: { username: string } };

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const data = await fetchPublicUser(params.username).catch(() => null);
  if (!data) {
    return { title: "Not found | Yokogushi" };
  }
  const name = data.profile.display_name ?? data.user.name ?? data.user.username;
  const description =
    data.profile.bio?.slice(0, 160) ??
    data.profile.headline ??
    `${name} のポートフォリオ`;
  const image = data.profile.avatar_url ?? data.user.avatar_url ?? undefined;
  return {
    title: `${name} | Yokogushi`,
    description,
    openGraph: {
      title: name,
      description,
      images: image ? [image] : undefined,
      type: "profile",
    },
  };
}

export default async function Page({ params }: Props) {
  const data = await fetchPublicUser(params.username).catch(() => null);
  if (!data) notFound();
  return <PublicPortfolioView data={data} />;
}
