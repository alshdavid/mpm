export type ModelType =
  | "Checkpoint"
  | "TextualInversion"
  | "Hypernetwork"
  | "AestheticGradient"
  | "LORA"
  | "Controlnet"
  | "Poses";

export type CommercialUseType =
  | "None"
  | "Image"
  | "Rent"
  | "Sell"
  | "RentCivit";

export type AvailabilityType = "Public" | "Private";

export type CivitAiModelsOptions = {
  limit?: number;
  page?: number;
  query?: string;
  tag?: string;
  username?: string;
  types?: Array<ModelType>;
  sort?: "Highest Rated" | "Most Downloaded" | "Newest";
  period?: "AllTime" | "Year" | "Month" | "Week" | "Day";
  rating?: number;
  favorites?: boolean;
  hidden?: boolean;
  primaryFileOnly?: boolean;
  allowNoCredit?: boolean;
  allowDerivatives?: boolean;
  allowDifferentLicenses?: boolean;
  allowCommercialUse?: CommercialUseType;
  nsfw?: boolean;
  supportsGeneration?: boolean;
  ids?: number[];
  baseModels?: string[];
};

export type CivitAiModel = {
  id: number;
  name: string;
  description: string;
  allowNoCredit: boolean;
  allowCommercialUse: Array<CommercialUseType>;
  allowDerivatives: true;
  allowDifferentLicense: boolean;
  type: ModelType;
  minor: boolean;
  sfwOnly: boolean;
  poi: boolean;
  nsfw: boolean;
  nsfwLevel: number;
  availability: AvailabilityType;
  cosmetic: null;
  supportsGeneration: true;
  stats: {
    downloadCount: number;
    favoriteCount: number;
    thumbsUpCount: number;
    thumbsDownCount: number;
    commentCount: number;
    ratingCount: number;
    rating: number;
    tippedAmountCount: number;
  };
  creator: {
    username: string;
    image: string;
  };
  tags: string[];
  modelVersions: Array<{
    id: number;
    index: number;
    name: string;
    baseModel: string;
    baseModelType: "Standard";
    publishedAt: string;
    availability: AvailabilityType;
    nsfwLevel: number;
    description?: string;
    trainedWords: [];
    stats: {
      downloadCount: number;
      ratingCount: number;
      rating: number;
      thumbsUpCount: number;
      thumbsDownCount: number;
    };
    downloadUrl: string;
    supportsGeneration: boolean;
    files: Array<{
      id: number;
      sizeKB: number;
      name: string;
      type: string;
      pickleScanResult: string;
      pickleScanMessage: string;
      virusScanResult: string;
      virusScanMessage: string;
      scannedAt: string;
      metadata: {
        format: string;
        size: string;
        fp: string;
      };
      hashes: {
        AutoV1: string;
        AutoV2: string;
        SHA256: string;
        CRC32: string;
        BLAKE3: string;
        AutoV3: string;
      };
      downloadUrl: string;
      primary: boolean;
    }>;
    images: Array<{
      id: number;
      url: string;
      nsfwLevel: number;
      width: number;
      height: number;
      hash: string;
      type: string;
      minor: boolean;
      poi: boolean;
      hasMeta: boolean;
      hasPositivePrompt: boolean;
      onSite: boolean;
      remixOfId?: string;
    }>;
  }>;
};

export type CivitAiModelsResponse = {
  items: Array<CivitAiModel>;
  metadata?: {
    nextPage?: string;
  };
};

export async function civitAiModel(
  id: number | string
): Promise<CivitAiModel> {
  const currentPage = civitAiCreateUrl(`https://civitai.com/api/v1/models/${id}`);
  return civitAiRest<CivitAiModel>(currentPage);
}

export async function* civitAiModels(
  options: CivitAiModelsOptions = {}
): AsyncIterable<CivitAiModelsResponse["items"][0]> {
  let currentPage = civitAiCreateUrl(
    "https://civitai.com/api/v1/models",
    options
  );
  while (currentPage) {
    // console.error(currentPage)
    const response = await civitAiRest<CivitAiModelsResponse>(currentPage);
    for (const model of response.items) {
      yield model;
    }
    if (!response?.metadata?.nextPage) {
      break;
    }
    console.log(response.metadata, response.items.length);
    // return
    currentPage = new URL(response.metadata.nextPage);
  }
}

function civitAiCreateUrl(
  url: string,
  options: Record<string, string | number | boolean | string[] | number[]> = {}
): URL {
  const parsed = new URL(url);
  if (process.env.CIVIT_TOKEN && !parsed.searchParams.has("token")) {
    parsed.searchParams.set("token", process.env.CIVIT_TOKEN);
  }

  for (const [key, value] of Object.entries(options)) {
    if (Array.isArray(value)) {
      for (const item of value) {
        parsed.searchParams.set(key, `${item}`);
      }
      continue;
    }
    parsed.searchParams.set(key, `${value}`);
  }

  return parsed;
}

async function civitAiRest<T = unknown>(url: URL): Promise<T> {
  const response = await globalThis.fetch(url);
  if (!response.ok) {
    throw new Error(response.statusText);
  }
  return response.json() as T;
}
