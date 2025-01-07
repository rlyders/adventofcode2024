package engine

import (
	"testing"
)

func Test_getSumOfDistancesOfListsText(t *testing.T) {
	type args struct {
		lists string
	}
	tests := []struct {
		name    string
		args    args
		want    SumOfDistancesResults
		wantErr bool
	}{
		{name: "",
			args: args{lists: `3   4
4   3
2   5
1   3
3   9
3   3`},
			want: SumOfDistancesResults{
				Sorted_location_pairs: []LocationPair{
					{
						Location_a: 1,
						Location_b: 3,
						Distance:   2,
					}, {
						Location_a: 2,
						Location_b: 3,
						Distance:   1,
					}, {
						Location_a: 3,
						Location_b: 3,
						Distance:   0,
					}, {
						Location_a: 3,
						Location_b: 4,
						Distance:   1,
					}, {
						Location_a: 3,
						Location_b: 5,
						Distance:   2,
					}, {
						Location_a: 4,
						Location_b: 9,
						Distance:   5,
					},
				},
				Sum_of_distances: 11,
				Elapsed_ms:       0,
				Elapsed_μs:       0,
			},
			wantErr: false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := GetSumOfDistancesOfListsText(tt.args.lists)
			if (err != nil) != tt.wantErr {
				t.Errorf("getSumOfDistancesOfListsText() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			if got.Sum_of_distances != tt.want.Sum_of_distances {
				t.Errorf("getSumOfDistancesOfListsText() = %v, want %v", got, tt.want)
			}
		})
	}
}
